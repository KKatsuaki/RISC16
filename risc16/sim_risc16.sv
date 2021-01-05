`timescale 1ns/1ps
`default_nettype none

module sim_risc16();
   localparam int     SIMULATION_CYCLES  = 1000;
   localparam real    CLOCK_FREQ_HZ      = 25 * 10**6; // 25MHz 
   localparam real    CLOCK_PERIOD_NS    = 10**9 / CLOCK_FREQ_HZ;
   logic              clk, rst;
   wire [15:0]        din, dout;
   wire [15:0]        addr;
   wire               oe, we;
   reg [7:0]          mem[0:65535];
   reg [23:0]         led;
   int                i;
   
   risc16 risc16_inst(.clk(clk), .rst(rst), .din(din), .dout(dout), 
                      .addr(addr), .oe(oe), .we(we));
   
   initial begin
      $readmemb("sim_risc16.mem", mem);
   end
   always_ff @(posedge clk) begin
      if (rst)
        led <= 24'h0;
      else if (we) begin
         if (addr == 16'h200)
           led[15:0] <= dout;
         else if (addr == 16'h202)
           led[23:16] <= dout[7:0];
         else begin
            mem[addr & 16'hfffe] <= dout[15:8];
            mem[addr | 16'h1] <= dout[7:0];
         end
      end
   end
   assign din = oe? {mem[addr & 16'hfffe], mem[addr | 16'h1]}: 16'hxxxx;

   initial begin
      clk <= 1'b0;
      repeat (SIMULATION_CYCLES) begin
       #(CLOCK_PERIOD_NS / 2.0) 
         clk <= 1'b1;
       #(CLOCK_PERIOD_NS / 2.0)
         clk <= 1'b0;
         print();
         if (risc16_inst.pc == 16'h1a)
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
      $write("==== clock: %1d", $rtoi($time / CLOCK_PERIOD_NS) - 1);      
      $write(" state: %s ====\n", risc16_inst.state.name);
      $write(" pc:%X ir:%B", risc16_inst.pc, risc16_inst.ir);
      $write(" treg1:%X treg2:%X", risc16_inst.treg1, risc16_inst.treg2);
      $write(" rdr:%X wdr:%X", risc16_inst.rdr, risc16_inst.wdr);
      $write(" oe:%B we:%B\n", risc16_inst.oe, risc16_inst.we);
      $write(" sbus1:%X sbus2:%X", risc16_inst.sbus1, risc16_inst.sbus2);
      $write(" dbus:%X addr:%X", risc16_inst.dbus, risc16_inst.addr);
      $write(" din:%X dout:%X", risc16_inst.din, risc16_inst.dout);
      $write(" led:%X\n", led);
      $write(" regs: %X", risc16_inst.reg_file_inst.register0);
      $write(" %X", risc16_inst.reg_file_inst.register1);
      $write(" %X", risc16_inst.reg_file_inst.register2);
      $write(" %X", risc16_inst.reg_file_inst.register3);
      $write(" %X", risc16_inst.reg_file_inst.register4);
      $write(" %X", risc16_inst.reg_file_inst.register5);
      $write(" %X", risc16_inst.reg_file_inst.register6);
      $write(" %X\n", risc16_inst.reg_file_inst.register7);
      for (i = 0; i < 40; i += 8) begin
         $write(" mem[%02x-%02x]:", i, i+7);
         $write(" %X %X %X %X",   mem[i],   mem[i+1], mem[i+2], mem[i+3]);
         $write(" %X %X %X %X\n", mem[i+4], mem[i+5], mem[i+6], mem[i+7]);
      end
      $write("\n");
   endtask
endmodule

`default_nettype wire
