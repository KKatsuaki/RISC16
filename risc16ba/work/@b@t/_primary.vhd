library verilog;
use verilog.vl_types.all;
entity BT is
    port(
        tag             : in     vl_logic_vector(7 downto 0);
        new_addr        : in     vl_logic_vector(15 downto 0);
        new_state       : in     vl_logic_vector(1 downto 0);
        we              : in     vl_logic_vector(1 downto 0);
        rst             : in     vl_logic;
        clk             : in     vl_logic;
        target          : out    vl_logic_vector(15 downto 0);
        state           : out    vl_logic_vector(1 downto 0);
        hit             : out    vl_logic
    );
end BT;
