`default_nettype none

`define ALU_THROUGH_AIN  4'b0000
...
  
module risc16p
  (
   input wire          clk,
   input wire          rst,
   input wire [15:0]   ddin,
   output logic [15:0] ddout,
   output wire [15:0]  daddr,
   output logic        doe,
   output logic        dwe,
   input wire [15:0]   idin,
   output wire [15:0]  iaddr,
   output wire         ioe
   );

   reg [15:0]          if_pc, if_ir;
   reg [15:0]          rf_pc, rf_ir, rf_immediate, rf_treg1, rf_treg2, ex_treg1;
   reg [15:0]          ex_ir, ex_result;
   logic [15:0]        alu_ain, alu_bin, alu_dout;
   logic [3:0]         alu_op;
   logic [15:0]        reg_file_dout1, reg_file_dout2;
   logic               reg_file_we, if_pc_we;

   // IF (Instruction Fetch) stage
   always_ff @(posedge clk) begin
      if (rst)
        if_ir <= 16'd0;
      else
        if_ir <= idin;
   end
   
   always_ff @(posedge clk) begin
      if (rst)
        if_pc <= 16'd0;
      else begin
         if (if_pc_we)
           if_pc <= ex_result;
         else
           if_pc <= if_pc + 16'd2;
      end
   end

   assign ioe = 1'b1;
   assign iaddr = if_pc;

   // RF (Register Fetch) stage
   reg_file reg_file_inst
     (
      .clk(clk),
      .rst(rst),
      .addr1(if_ir[10:8]),
      .addr2(if_ir[7:5]),
      .addr3(ex_ir[10:8]),
      .din(ex_result),
      .dout1(reg_file_dout1),
      .dout2(reg_file_dout2),
      .we(reg_file_we)
      );

   always_ff @(posedge clk) begin
      if (rst)
        rf_ir <= 16'd0;
      else
        rf_ir <= if_ir;
   end

   always_ff @(posedge clk) begin
      if (rst) 
        rf_treg1 <= 16'd0;
      else 
        rf_treg1 <= reg_file_dout1;
   end 

   always_ff @(posedge clk) begin
      if (rst) 
        rf_treg2 <= 16'd0;
      else 
        rf_treg2 <= reg_file_dout2;
   end 

   always_ff @(posedge clk) begin
      if (rst)
        rf_immediate <= 16'd0;
      else begin
         if (if_ir[15] == 1'b0) begin // Register, Memory, or Immediate type
            if (if_ir[14:11] == `ALU_ADD) begin // ADDI
               if (if_ir[7] == 1'b0) // sign extension
                 rf_immediate <= {8'h00, if_ir[7:0]};
               else
                 rf_immediate <= {8'hff, if_ir[7:0]};
            end
            else // zero extension
              rf_immediate <= {8'h00, if_ir[7:0]};
         end
         else begin // Branch or Jump type
            if (if_ir[14] == 1'b0) begin // Branch type
               if (if_ir[7] == 1'b0) // sign extension
                 rf_immediate <= {8'h00, if_ir[7:0]};
               else
                 rf_immediate <= {8'hff, if_ir[7:0]};
            end
            else begin // Jump type
               if (if_ir[10] == 1'b0) // sign extension
                 rf_immediate <= {5'h00, if_ir[10:0]};
               else
                 rf_immediate <= {5'h1f, if_ir[10:0]};
            end
         end
      end
   end

   always_ff @(posedge clk) begin
      if (rst)
        rf_pc <= 16'd0;
      else
        rf_pc <= if_pc;
   end
   
   // EX (Execution) stage
   alu16 alu16_inst
     (
      .ain(alu_ain), 
      .bin(alu_bin), 
      .op(alu_op), 
      .dout(alu_dout)
      );

   always_ff @(posedge clk) begin
      if (rst)
        ex_ir <= 16'd0;
      else
        ex_ir <= rf_ir;
   end

   always_ff @(posedge clk) begin
      if (rst)
        ex_treg1 <= 16'd0;
      else
        ex_treg1 <= rf_treg1;
   end
   
   always_ff @(posedge clk) begin
      if (rst)
        ex_result <= 16'd0;
      else begin
         if (rf_ir[15:11] == 5'd0 && rf_ir[4] == 1'b1 && rf_ir[0] == 1'b1) 
           ex_result <= ddin; // LD
         else
           ex_result <= alu_dout;
      end
   end

   assign daddr = rf_treg2;
    
   always_comb begin // alu_ain, alu_bin, alu_op, dout, doe, dwe
      ...
   end
   
   // WB (Write Back) stage
   always_comb begin // if_pc_we, reg_file_we
     ...
   end
endmodule

module reg_file
  (
   input wire          clk, rst,
   input wire [2:0]    addr1, addr2, addr3,
   input wire [15:0]   din,
   output logic [15:0] dout1, dout2,
   input wire          we
   );
   
   reg [15:0]          register0, register1;
   reg [15:0]          register2, register3;
   reg [15:0]          register4, register5;
   reg [15:0]          register6, register7;
   
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
        case (addr3)
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

...

`default_nettype wire
