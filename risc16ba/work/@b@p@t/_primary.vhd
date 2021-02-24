library verilog;
use verilog.vl_types.all;
entity BPT is
    port(
        ex_pc           : in     vl_logic_vector(15 downto 0);
        target          : in     vl_logic_vector(15 downto 0);
        cur_pc          : in     vl_logic_vector(15 downto 0);
        we              : in     vl_logic;
        rst             : in     vl_logic;
        clk             : in     vl_logic;
        next_pc         : out    vl_logic_vector(15 downto 0);
        pred            : out    vl_logic_vector(1 downto 0)
    );
end BPT;
