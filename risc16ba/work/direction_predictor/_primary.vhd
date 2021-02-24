library verilog;
use verilog.vl_types.all;
entity direction_predictor is
    port(
        clk             : in     vl_logic;
        rst             : in     vl_logic;
        actually_taken  : in     vl_logic;
        state           : in     vl_logic_vector(1 downto 0);
        \next\          : out    vl_logic_vector(1 downto 0)
    );
end direction_predictor;
