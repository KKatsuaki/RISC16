library verilog;
use verilog.vl_types.all;
entity BTB is
    port(
        new_target      : in     vl_logic_vector(15 downto 0);
        tag_r           : in     vl_logic_vector(9 downto 0);
        tag_w           : in     vl_logic_vector(9 downto 0);
        taken           : in     vl_logic;
        rst             : in     vl_logic;
        clk             : in     vl_logic;
        we              : in     vl_logic;
        pred            : out    vl_logic;
        target          : out    vl_logic_vector(15 downto 0)
    );
end BTB;
