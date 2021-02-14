library verilog;
use verilog.vl_types.all;
entity risc16f is
    port(
        clk             : in     vl_logic;
        rst             : in     vl_logic;
        ddin            : in     vl_logic_vector(15 downto 0);
        ddout           : out    vl_logic_vector(15 downto 0);
        daddr           : out    vl_logic_vector(15 downto 0);
        doe             : out    vl_logic;
        dwe             : out    vl_logic;
        idin            : in     vl_logic_vector(15 downto 0);
        iaddr           : out    vl_logic_vector(15 downto 0);
        ioe             : out    vl_logic
    );
end risc16f;
