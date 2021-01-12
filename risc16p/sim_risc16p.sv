`timescale 1ns/1ps
`default_nettype none

module sim_risc16p();
   localparam integer SIMULATION_CYCLES  = 1000;
   localparam real    CLOCK_FREQ_HZ      = 25 * 10**6; // 25MHz 
   localparam real    CLOCK_PERIOD_NS    = 10**9 / CLOCK_FREQ_HZ;
   logic              clk, rst;
   wire [15:0] 	      ddin, ddout, idin;
   wire [15:0] 	      daddr, iaddr;
   wire               doe, dwe, ioe;
   reg [7:0]          mem[0:65535];
   wire [23:0] 	      led;
   reg [7:0] 	      led_0, led_1, led_2;
   integer            i;
   
   risc16p risc16p_inst(.clk(clk), .rst(rst), .ddin(ddin), .ddout(ddout), 
			.daddr(daddr), .doe(doe), .dwe(dwe),
			.idin(idin), .iaddr(iaddr), .ioe(ioe));
   
   initial begin
      $readmemb("sim_risc16p.mem", mem);
   end
   always_ff @(posedge clk) begin
      if (rst) begin
         led_0 <= 8'h0;
	 led_2 <= 8'h0;
      end
      else if (dwe) begin
         if (daddr == 16'h200) 
           led_0 <= ddout[7:0];
         else if (daddr == 16'h202)
           led_2 <= ddout[7:0];
         else begin
            mem[daddr | 16'h1] <= ddout[7:0];
         end
      end
   end
   always_ff @(posedge clk) begin
      if (rst) begin
         led_1 <= 8'h0;
      end
      else if (dwe) begin
         if (daddr == 16'h200) 
           led_1 <= ddout[15:8];
         else begin
	    mem[daddr & 16'hfffe] <= ddout[15:8];
         end
      end
   end
   assign led = {led_2, led_1, led_0};
   
   assign ddin = doe? {mem[daddr & 16'hfffe], mem[daddr | 16'h1]}: 16'hxxxx;
   assign idin = ioe? {mem[iaddr & 16'hfffe], mem[iaddr | 16'h1]}: 16'hxxxx;
   
   initial begin
      clk <= 1'b0;
      repeat (SIMULATION_CYCLES) begin
       #(CLOCK_PERIOD_NS / 2.0) 
         clk <= 1'b1;
       #(CLOCK_PERIOD_NS / 2.0)
         clk <= 1'b0;
         print();
	 if (risc16p_inst.if_pc == 16'hff)
	   $finish;
      end
      $finish;
   end
   
   initial begin
      rst <= 1'b1;
    #(CLOCK_PERIOD_NS)
      rst <= 1'b0;      
   end

   task print(); 
      int i;
      $write("==== clock: %1d ====\n", $rtoi($time / CLOCK_PERIOD_NS) - 1);  
      $write(" if_pc:%X if_ir:%B\n", 
	     risc16p_inst.if_pc, risc16p_inst.if_ir);
      $write(" rf_pc:%X rf_ir:%B rf_treg1:%X rf_treg2:%X rf_immediate:%X\n",
	     risc16p_inst.rf_pc, risc16p_inst.rf_ir,
	     risc16p_inst.rf_treg1, risc16p_inst.rf_treg2,
	     risc16p_inst.rf_immediate);
      $write(" ex_ir:%B ex_result:%X\n",
	     risc16p_inst.ex_ir, risc16p_inst.ex_result);
      $write(" daddr:%X ddin:%X ddout:%X doe:%B dwe:%B\n",
	     risc16p_inst.daddr, risc16p_inst.ddin, risc16p_inst.ddout, 
	     risc16p_inst.doe, risc16p_inst.dwe);
      $write(" iaddr:%X idin:%X ioe:%B\n",
	     risc16p_inst.iaddr, risc16p_inst.idin, risc16p_inst.ioe);
      $write(" alu_ain:%X alu_bin:%X alu_op:%B reg_file_we:%B if_pc_we:%B",
	     risc16p_inst.alu_ain, risc16p_inst.alu_bin,
	     risc16p_inst.alu_op, risc16p_inst.reg_file_we,
	     risc16p_inst.if_pc_we);
      $write(" led:%X\n", led);
      $write(" regs: %X", risc16p_inst.reg_file_inst.register0);
      $write(" %X", risc16p_inst.reg_file_inst.register1);
      $write(" %X", risc16p_inst.reg_file_inst.register2);
      $write(" %X", risc16p_inst.reg_file_inst.register3);
      $write(" %X", risc16p_inst.reg_file_inst.register4);
      $write(" %X", risc16p_inst.reg_file_inst.register5);
      $write(" %X", risc16p_inst.reg_file_inst.register6);
      $write(" %X\n", risc16p_inst.reg_file_inst.register7);
      for (i = 0; i < 56; i += 8) begin
         $write(" mem[%02x-%02x]:", i, i+7);
         $write(" %X %X %X %X",   mem[i],   mem[i+1], mem[i+2], mem[i+3]);
         $write(" %X %X %X %X\n", mem[i+4], mem[i+5], mem[i+6], mem[i+7]);
      end
      $write("\n");
   endtask
endmodule

`default_nettype wire
