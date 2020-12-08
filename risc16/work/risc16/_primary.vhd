library verilog;
use verilog.vl_types.all;
entity risc16 is
    port(
        clk             : in     vl_logic;
        rst             : in     vl_logic;
        din             : in     vl_logic_vector(15 downto 0);
        dout            : out    vl_logic_vector(15 downto 0);
        addr            : out    vl_logic_vector(15 downto 0);
        oe              : out    vl_logic;
        we              : out    vl_logic
    );
end risc16;
