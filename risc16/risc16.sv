`default_nettype none
  typedef enum {IF, RF, EX, WB} state_t;
`define THROUGH_AIN 4'b0000
`define THROUGH_BIN 4'b0001
`define NOT_B 4'b0010
`define XOR 4'b0011
`define ADD 4'b0100
`define SUB 4'b0101
`define LEFT_SHIFT_BIN_8 4'b0110
`define LEFT_SHIFT_BIN_1 4'b1000
`define RIGHT_SHIFT_BIN_1 4'b1001
`define AND 4'b1010
`define OR 4'b1011

module risc16
  (
   input wire          clk,
   input wire          rst,
   input wire [15:0]   din,
   output logic [15:0] dout,
   output logic [15:0] addr,
   output logic        oe,
   output logic        we
   );
   
   state_t state;
   reg [15:0] 	       pc, ir, rdr, wdr, treg1, treg2;
   logic [15:0]        sbus1, sbus2;                   // source buses
   wire [15:0] 	       dbus;                           // destination bus
   logic [3:0] 	       op;                             // ALU operation
   wire [15:0] 	       reg_file_dout1, reg_file_dout2; // register file outputs
   logic               pc_we, ir_we, rdr_we, wdr_we, treg_we, reg_file_we;

   alu16 alu16_inst
     (
      .ain(sbus1), 
      .bin(sbus2), 
      .op(op), 
      .dout(dbus)
      );
   
   reg_file reg_file_inst
     (
      .clk(clk),
      .rst(rst),
      .addr1(ir[10:8]),
      .addr2(ir[7:5]),
      .din(dbus),
      .dout1(reg_file_dout1),
      .dout2(reg_file_dout2),
      .we(reg_file_we)
      );

   // state control
   always_ff @(posedge clk) begin
      if (rst)
        state <= IF;
	else begin
           case (state)
             IF: state <= RF;
             RF: state <= EX;
             EX: if ( ir[4] == 1'b1 && ir[15:11] == 5'b00000) 
	       // load or store instructions
               state <= WB;
             else
               state <= IF;
             WB: state <= IF;
           endcase
	end 
   end

   // program counter
   always_ff @(posedge clk) begin
      if (rst)
        pc <= 16'h0;
      else if (pc_we)
        pc <= dbus;
   end
   
   // instruction register   
   always_ff @(posedge clk) begin
      if (rst)
        ir <= 16'd0;
      else if (ir_we)
        ir <= din;
   end

   // read data register   
   always_ff @(posedge clk) begin
      if (rst)
        rdr <= 16'd0;
      else if (rdr_we)
        rdr <= din;
   end

   // write data register      
   always_ff @(posedge clk) begin 
      if (rst)
        wdr <= 16'd0;
      else if (wdr_we)
        wdr <= dbus;
   end
   
   // temporal output registers from register file
   always_ff @(posedge clk) begin
      if (rst) begin
         treg1 <= 16'h0;
         treg2 <= 16'h0;
      end
      else if (treg_we) begin
         treg1 <= reg_file_dout1;
         treg2 <= reg_file_dout2;
      end
   end

   // behavior of state machine
   /*
    `we` stands for write enable
    `oe` stands for output enable
    i.e. if `we` or `oe` are 1, write to the reg, pc, mem or etc.
    */
   always_comb begin
      case (state)
        IF: begin /* instruction fetch */
           addr <= pc;
           sbus1 <= pc;
           sbus2 <= 16'h0002;
           op <= `ADD;
           dout <= 16'bx;
           pc_we <= 1'b1;
           ir_we <= 1'b1;
           rdr_we <= 1'b0;
           wdr_we <= 1'b0;
           treg_we <= 1'b0;
           reg_file_we <= 1'b0;
           oe <= 1'b1;
           we <= 1'b0;
        end

        RF: begin /* registor fetch */
           addr <= 16'bx;
           sbus1 <= 16'bx;
           sbus2 <= 16'bx;
           op <= 16'bx;
           dout <= 16'bx;
           pc_we <= 1'b0;
           ir_we <= 1'b0;
           rdr_we <= 1'b0;
           wdr_we <= 1'b0;
           treg_we <= 1'b1;
           reg_file_we <= 1'b0;
           oe <= 1'b0;
           we <= 1'b0;
        end

        EX: begin /* execution */
	   addr <= treg2;
	   if (ir[15] == 1'b1)
	     sbus1 <= pc;
	   else		      
	     sbus1 <= treg1;

	   // head of branch
	   if(ir[15] == 1'b1) begin
	      if (ir[14] != 1'b1) // branch
		sbus2 <= {{8{ir[7]}},ir[7:0]};
	      else // jmp
		sbus2 <= {{5{ir[10]}},ir[10:0]};		
	      op <= `ADD;
   	   end
	   else 
	     begin
		if(ir[15:11]==5'b00000 && ir[4] == 0) begin// registor
		   sbus2 <= treg2;
		   op <= ir[3:0];
		end else begin// other
		   sbus2 <= {{8{ir[7]}},ir[7:0]};
		   if(ir[15] == 1'b0 && ir[14:11]!= 4'b0000) //immediate
		     op <= ir[14:11];
		   else
		     op <= `THROUGH_AIN; // st value. other's value are don't care
		end
	     end // else: !if(ir[15] == 1'b1)

	   dout <= 16'bx;

	   if(ir[15] == 1'b1)begin
	      if(ir[14] == 1'b1)
		pc_we <= 1'b1;
	      else begin
		 case(ir[12:11])
		   2'b00 : begin // BNEZ
		      if(treg1 != 16'b0)
			pc_we <= 1'b1;
		      else
			pc_we<= 1'b0;
		   end
		   2'b01 : begin // BEQZ
 		      if(treg1 == 16'b0)
			pc_we <= 1'b1;
		      else
			pc_we<= 1'b0;
		   end
		   2'b10:begin
 		      if(treg1[15] == 1'b1)
			pc_we <= 1'b1;
		      else
			pc_we<= 1'b0;
		   end
		   2'b11 : begin
 		      if(treg1[15] == 1'b0)
			pc_we <= 1'b1;
		      else
			pc_we<= 1'b0;
		   end
		 endcase // case (ir[12:11])
		 end // else: !if(ir[14] == 1'b1)
	   end // if (ir[15] == 1'b1)

	   else
	     pc_we <= 1'b0;
	   ir_we <= 1'b0;

	   if(ir[15:11] == 5'b00000 && ir[4] == 1'b1 && ir[0] == 1'b1) // if ld
	     rdr_we = 1'b1;
	   else
	     rdr_we = 1'b0;
	   
	   if(ir[15:11] == 5'b00000 && ir[4] == 1'b1 && ir[0] == 1'b0) // if st
	     wdr_we = 1'b1;
	   else
	     wdr_we = 1'b0;

	   treg_we <= 1'b0;

	   //if((ir[15:11] == 5'b00000 && ir[4] == 1'b0) || (ir[15] == 1'b0 && ir[14:11] != 4'b0000))
	   if(ir[15] == 1'b0)	   
	     reg_file_we <= 1'b1;
	   else
	     reg_file_we <= 1'b0;

	   if(ir[15:11] == 5'b00000 && ir[4] == 1'b1 && ir[0] == 1'b1) // if ld
	     oe <= 1'b1;
	   else
	     oe <= 1'b0;

	   we <= 1'b0;
        end // case: EX

	WB: begin 
           addr <= treg2;
	   sbus1 <= 16'bx;
	   sbus2 <= rdr;
	   op <= `THROUGH_BIN;
	   dout <= wdr ;
	   oe <= 1'b0;
	   pc_we <= 1'b0;	      
	   ir_we <= 1'b0;
	   rdr_we <= 1'b0;
	   wdr_we <= 1'b0;
	   treg_we <= 1'b0;
	   if(ir[0] == 1'b1) begin//ld
	      reg_file_we <= 1'b1;
	      we <= 1'b0;
	   end else begin // if (ir[0]) st
	      reg_file_we <= 1'b0;
	      we <= 1'b1;
	   end // else: !if(ir[0])
	end // case: WB
		 endcase 
	      end
	      endmodule

module reg_file
  (
   input wire          clk, rst,
   input wire [2:0]    addr1, addr2,
   input wire [15:0]   din,
   output logic [15:0] dout1, dout2,
   input wire          we
   );
   
   reg [15:0] 	       register0, register1;
   reg [15:0] 	       register2, register3;
   reg [15:0] 	       register4, register5;
   reg [15:0] 	       register6, register7;
   
   always_comb begin
      case (addr1)
        3'h0: dout1 <= register0;
        3'h1: dout1 <= register1;
        3'h2: dout1 <= register2;
        3'h3: dout1 <= register3;
        3'h4: dout1 <= register4;
        3'h5: dout1 <= register5;
        3'h6: dout1 <= register6;
        3'h7: dout1 <= register7;
      endcase 
   end
   
   always_comb begin
      case (addr2)
        3'h0: dout2 <= register0;
        3'h1: dout2 <= register1;
        3'h2: dout2 <= register2;
        3'h3: dout2 <= register3;
        3'h4: dout2 <= register4;
        3'h5: dout2 <= register5;
        3'h6: dout2 <= register6;
        3'h7: dout2 <= register7;
      endcase 
   end

   always_ff @(posedge clk)
     if (rst) begin
        register0 <= 16'h0;
        register1 <= 16'h0;
        register2 <= 16'h0;
        register3 <= 16'h0;
        register4 <= 16'h0;
        register5 <= 16'h0;
        register6 <= 16'h0;
        register7 <= 16'h0;
     end
     else if (we) begin
        case (addr1)
          3'h0: register0 <= din;
          3'h1: register1 <= din;
          3'h2: register2 <= din;
          3'h3: register3 <= din;
          3'h4: register4 <= din;
          3'h5: register5 <= din;
          3'h6: register6 <= din;
          3'h7: register7 <= din;
        endcase
     end
endmodule 


module alu16
  (
   input wire [15:0] ain,bin,
   input wire [3:0] op,
   output logic [15:0] dout
   );

   always_comb begin
      case (op)
	`THROUGH_AIN: dout <= ain;
	`THROUGH_BIN: dout <= bin;
	`NOT_B: dout <= ~bin;
	`XOR: dout <= ain ^ bin;
	`ADD : dout <= ain + bin;
	`SUB : dout <= ain - bin;
	`LEFT_SHIFT_BIN_8: dout <= bin << 8;
	`LEFT_SHIFT_BIN_1 : dout <= bin << 1;
	`RIGHT_SHIFT_BIN_1 : dout <= bin >> 1;
	`AND : dout <= ain & bin;
	`OR:dout<= ain | bin;
	default : dout <= 16'bx;
      endcase // case (op)
   end // always_comb begin
endmodule
`default_nettype wire
