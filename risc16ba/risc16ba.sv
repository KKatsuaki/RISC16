`default_nettype none                 
`define ALU_THROUGH_AIN 4'b0000           
`define ALU_THROUGH_BIN 4'b0001           
`define ALU_NOT_B 4'b0010                 
`define ALU_XOR 4'b0011                   
`define ALU_ADD 4'b0100                   
`define ALU_SUB 4'b0101                   
`define ALU_LEFT_SHIFT_BIN_8 4'b0110      
`define ALU_LEFT_SHIFT_BIN_1 4'b1000      
`define ALU_RIGHT_SHIFT_BIN_1 4'b1001     
`define ALU_AND  4'b1010                   
`define ALU_OR   4'b1011

`define NOP 16'b0000_0000_0000_0000

module risc16ba
  (
   input wire          clk,
   input wire          rst,
   input wire [15:0]   ddin,
   output logic [15:0] ddout,
   output wire [15:0]  daddr,
   output logic        doe,
   output logic        dwe0, dwe1,
   input wire [15:0]   idin,
   output wire [15:0]  iaddr,
   output wire         ioe
   );

   // objects for if stage
   reg [15:0]          if_pc, if_ir;
   logic 	       if_pc_we; 

   // register for rf stage
   reg [15:0]          rf_pc, rf_ir, rf_imm, rf_treg1, rf_treg2;

   // registor for ex stage
   reg [15:0]          ex_ir, ex_result, ex_forwarding, ex_pc, ex_treg1;

   // wire for alu
   logic [15:0]        alu_ain, alu_bin, alu_dout;
   logic [3:0]         alu_op;

   // wire for register
   logic [15:0]        reg_dout1, reg_dout2;
   logic               reg_we;
   
   // IF stage

   // if_ir
   always_ff @(posedge clk)
     if_ir <= rst ? 16'h0 : idin;

   // should BPT later
   always_ff @(posedge clk) begin
      if(rst)
	if_pc <= 16'h0;
      else
	if_pc <= if_pc_we? ex_result : if_pc + 16'h2;
   end

   assign iaddr = if_pc;
   assign ioe = 1'b1;

   // RF stage
   always_ff @(posedge clk)	   
     rf_ir <= rst ? 16'h0 : if_ir;

   always_ff @(posedge clk)        
     rf_pc <= rst ? 16'h0 : if_pc; 

   reg_file reg_file_inst
     (
      .clk(clk),
      .rst(rst),
      .addr1(if_ir[10:8]),
      .addr2(if_ir[7:5]),
      .addr3(ex_ir[10:8]),
      .dout1(reg_dout1),
      .dout2(reg_dout2),
      .din(ex_result),
      .we(reg_we)
      );
   
   //rf_imm
   always_ff @(posedge clk) begin
      if(rst)
        rf_imm <= 16'h0;
      else if(if_ir[15:14] == 2'b11) // JMP 
        rf_imm <= {{5{if_ir[10]}}, if_ir[10:0]}; // sign extention 
      else if((~if_ir[15] && if_ir[14:11] == `ALU_ADD) || if_ir[15:14] == 2'b10) // ADDI or Branch
        rf_imm <= {{8{if_ir[7]}}, if_ir[7:0]}; // sign extention
      else
        rf_imm <= {8'd0,if_ir[7:0]}; // zero extention
   end

   // treg1
   always_ff @(posedge clk) begin
      if(rst)
	rf_treg1 <= 16'd0;
      else if(if_ir[10:8] == ex_ir[10:8] || if_ir[10:8] == rf_ir[10:8]) begin
	 if(rf_ir[10:8] == if_ir[10:8]) begin
	    if(rf_ir != `NOP && rf_ir[15] == 1'b0 && ~(rf_ir[15:11] == 5'b00000 && rf_ir[4:0] == 5'b10000))
	      rf_treg1 <= ex_forwarding;
	    else
	      rf_treg1 <= reg_dout1;
	 end else if(reg_we) 
	   rf_treg1 <= ex_result;
	 else
	   rf_treg1 <= reg_dout1;
      end else // if (if_ir[10:8] == ex_ir[10:8] || if_ir[10:8] == rf_ir[10:8])
	rf_treg1 <= reg_dout1;
   end // always_ff @ (posedge clk)

   // treg2
   always_ff @(posedge clk) begin
      if(rst)
        rf_treg2 <= 16'd0;
      else if(if_ir[7:5] == ex_ir[10:8] || if_ir[7:5] == rf_ir[10:8]) begin
         if(rf_ir[7:5] == if_ir[10:8]) begin
            if(rf_ir != `NOP && rf_ir[15] == 1'b0 && ~(rf_ir[15:11] == 5'b00000 && rf_ir[4:0] == 5'b10000))
              rf_treg2 <= ex_forwarding;
            else
              rf_treg2 <= reg_dout2;
         end else if(reg_we)                                                                               
           rf_treg2 <= ex_result;
         else
           rf_treg2 <= reg_dout2;
      end else // if (if_ir[7:5] == ex_ir[10:8] || if_ir[7:5] == rf_ir[10:8])
        rf_treg2 <= reg_dout2;
   end // always_ff @ (posedge clk)

   // EX stage
   alu16 alu16_inst
     (
      .ain(alu_ain), 
      .bin(alu_bin), 
      .op(alu_op),   
      .dout(alu_dout)      
      );

   always_ff @(posedge clk)
     ex_ir <= rst ? 16'h0 : rf_ir;

   always_ff @(posedge clk)	       
     ex_treg1 <= rst? 16'h0 : rf_treg1;

   always_ff @(posedge clk)
     ex_pc <= rf_pc;
   
   always_ff @(posedge clk)
     ex_result <= rst? 16'h0 : ex_forwarding;

   always_comb begin
      if(rf_ir[15:11] == 5'b00000 && rf_ir[4])
	case(rf_ir[3:0])
	  4'b0001 : ex_forwarding = ddin;
	  4'b0011 : ex_forwarding = rf_treg2[0] ? {8'b0,ddin[7:0]}:{8'b0,ddin[15:8]};
	  default : ex_forwarding = alu_dout;
	endcase // case (rf_ir[3:0])
      else
	ex_forwarding = alu_dout;
   end

   assign daddr = rf_treg2;
   // alu_ain, alu_bin, alu_op, dout, doe, dwe
   /*
    |      |reg       |ST      |LD|IMM         |BR and JMP          
    alu_ain|rf_treg1  |x       |x |rf_treg1    |rf_pc        
    alu_bin|rf_treg2  |x       |x |rf_imm      |rf_imm
    alu_op |rf_ir[3:0]|x       |x |rf_ir[14:11]|`ALU_ADD         
    ddout  |x         |rf_treg1|x |x           |x            
    doe    |0         |0       |1 |0           |0            
    dwe1   |0         |1       |0 |0           |0            
    dwe2   |0         |1       |0 |0           |0
    */

   always_comb begin
      alu_ain = rf_ir[15]? rf_pc:rf_treg1;
      alu_bin = (rf_ir[15:11]==5'b0)? rf_treg2 : rf_imm;

      if(rf_ir[15])
        alu_op = `ALU_ADD;
      else
        alu_op = (rf_ir[15:11]==5'b0)? rf_ir[3:0] : rf_ir[14:11];

      doe = (rf_ir[4]&&rf_ir[0])? 1'b1:1'b0;

      if(rf_ir[15:11] == 5'b0 && rf_ir[4]) begin
         case(rf_ir[3:0])                       
           4'h0 : begin // st                   
              dwe1  = 1'b1;                     
              dwe0  = 1'b1;                     
              ddout <= rf_treg1;                
           end                                  
           4'h1 : begin //ld                    
              dwe1 = 1'b0;                      
              dwe0 = 1'b0;                      
              ddout <= 16'hx;                   
           end                                  
           4'h2 : begin // SBU                  
              if(rf_treg2[0]) begin             
                 dwe0  =  1'b0;                 
                 dwe1  =  1'b1;                 
                 ddout <= {8'b0, rf_treg1[7:0]};
              end else begin                    
                 dwe0  = 1'b1;                  
                 dwe1  = 1'b0;                  
                 ddout <= {rf_treg1[7:0],8'b0}; 
              end                               
           end                                  
           4'h3 : begin // LBU                  
              dwe1 = 1'b0;                      
              dwe0 = 1'b0;                      
              ddout <= 16'hx;                   
           end                                  
           default: begin                       
              dwe1 = 1'b0;                      
              dwe0 = 1'b0;                      
              ddout <= 16'hx;                   
           end                                  
         endcase // case (rf_ir[3:0])           
      end // if (rf_ir[15:11 == 5'b0 && rf_ir[4])
      else begin // imm                            
         ddout <= 16'bx;                           
         dwe0 <= 1'b0;                             
         dwe1 <= 1'b0;                             
      end // else: !if(rf_ir[15:11 == 5'b0 && rf_ir[4])
   end // always_comb                                 


   // WB stage
   always_comb begin // if_pc_we, reg_file_we
      /*
       |          |reg          |ST |LD |IMM|BR                           |JMP                                         
       if_pc_we   |0            |0  |0  |0  |1(if satsfy *check ex_treg1) |1
       reg_file_we|1(if not NOP)|0  |1  |1  |0                            |0
       */
      if(ex_ir[15] == 1'b0)  begin // reg, mem, imm
         if_pc_we = 1'b0;
         if(ex_ir == 16'b0 /*NOP*/ || (ex_ir[14:11] == 4'b0 && ex_ir[4] == 1'b1 && ex_ir[0] == 1'b0) /*ST*/)
           reg_we = 1'b0;
         else
           reg_we = 1'b1;
      end
      else begin // BR and JMP
         reg_we = 1'b0;

         if(ex_ir[14] == 1'b1) //JMP
           if_pc_we = 1'b1;
         else begin
            case(ex_ir[12:11])
              2'b00 : begin // BNEZ
                 if(ex_treg1 != 16'b0)
                   if_pc_we = 1'b1;
                 else
                   if_pc_we = 1'b0;
              end
              2'b01 : begin // BEQZ
                 if(ex_treg1 == 16'b0)
                   if_pc_we = 1'b1;
                 else
                   if_pc_we = 1'b0;
              end
              2'b10:begin
                 if(ex_treg1[15])
                   if_pc_we = 1'b1;
                 else
                   if_pc_we = 1'b0;
              end
              2'b11 : begin
                 if(ex_treg1[15])
                   if_pc_we = 1'b0;
                 else
                   if_pc_we = 1'b1;
              end
            endcase // case (ex_ir[12:11])
         end // else: !if(ex_ir[14] == 1'b1)
      end // else: !if(rf_ir[15] == 1'b0)
   end // always_comb
endmodule // risc16ba

module reg_file
  (
   input wire          clk, rst,
   input wire [2:0]    addr1, addr2, addr3,
   input wire [15:0]   din,
   output logic [15:0] dout1, dout2,
   input wire          we
   );
   
   reg [15:0]          register[7:0];
   integer 	       i;
   
   always_comb begin
      dout1 <= register[addr1];
   end
   
   always_comb begin
      dout2 <= register[addr2];
   end

   always_ff @(posedge clk)
     if (rst) begin
	for(i = 0; i < 8; i++)
	  register[i] <= 16'h0;
     end else if (we) 
       register[addr3] <= din;
endmodule // reg_file

module alu16
  (
   input wire [15:0]   ain,bin, 
   input wire [3:0]    op, 
   output logic [15:0] dout                   
   );                                         
   
   always_comb begin                          
      case (op)                               
        `ALU_THROUGH_AIN      : dout <= ain;      
        `ALU_THROUGH_BIN      : dout <= bin;      
        `ALU_NOT_B            : dout <= ~bin;     
        `ALU_XOR              : dout <= ain ^ bin;
        `ALU_ADD              : dout <= ain + bin;
        `ALU_SUB              : dout <= ain - bin;
        `ALU_LEFT_SHIFT_BIN_8 : dout <= bin << 8; 
        `ALU_LEFT_SHIFT_BIN_1 : dout <= bin << 1; 
        `ALU_RIGHT_SHIFT_BIN_1: dout <= bin >> 1; 
        `ALU_AND              : dout <= ain & bin;
        `ALU_OR               : dout <= ain | bin;
        default           : dout <= 16'bx;    
      endcase // case (op)                    
   end // always_comb begin                   
endmodule // alu16

module BPT
  (
   input wire [15:0]  ex_pc,
   input wire [15:0]  target,
   input wire [15:0]  cur_pc,
   input wire         we, rst, clk, 
   output logic [15:0] next_pc,
   output logic [1:0 ] pred
   );

   reg [18:0]         btb[1023:0];
   /*
    | flag 1 | addr 16| state 2| 
    */

   integer            i;
   
   always_ff @(posedge clk) begin
      if (rst) begin
         for(i = 0; i < 1023; i += 1) 
           btb[i] <= 18'h0;
      end 
      else if(we) 
        btb[ex_pc[11:2]] <= {1'd1,target,2'b0};
   end

   always_comb begin
      next_pc <= btb[cur_pc[11:2]][17:2];
      pred <= btb[cur_pc][1:0];
   end

endmodule // BPT
