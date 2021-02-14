set_time_format -unit ns -decimal_places 3
create_clock -name {clk} -period 41.666 -waveform { 0.000 20.833 } [get_ports {clk_ext_in}]
create_clock -name {usb_clk} -period 41.666 -waveform { 0.000 20.833 } [get_ports {clk_usb_in}]
create_clock -name {vclk} -period 41.666
create_generated_clock -source {clk_generator_inst|altpll_component|pll|inclk[0]} -duty_cycle 50.00 -name {processor_clk} {clk_generator_inst|altpll_component|pll|clk[0]}
create_generated_clock -source {clk_generator_inst|altpll_component|pll|inclk[0]} -phase -90.00 -duty_cycle 50.00 -name {write_clk} {clk_generator_inst|altpll_component|pll|clk[1]}

set_false_path -from [get_ports {n_rst_ext_in}]
set_false_path -from [get_ports {sw_in*}]
set_output_delay            -clock vclk -max 0 [get_ports {seg_*}]
set_output_delay -add_delay -clock vclk -min 0 [get_ports {seg_*}]
set_output_delay            -clock vclk -max 0 [get_ports {led*}]
set_output_delay -add_delay -clock vclk -min 0 [get_ports {led*}]

set_output_delay            -clock vclk -max 4 [get_ports {mem_a_n_we}]
set_output_delay -add_delay -clock vclk -min 0 [get_ports {mem_a_n_we}]
set_output_delay            -clock vclk -max 4 [get_ports {mem_a_n_oe}]
set_output_delay -add_delay -clock vclk -min 0 [get_ports {mem_a_n_oe}]
set_output_delay            -clock vclk -max 4 [get_ports {mem_a_n_ce}]
set_output_delay -add_delay -clock vclk -min 0 [get_ports {mem_a_n_ce}]
set_output_delay            -clock vclk -max 4 [get_ports {mem_a_n_lb}]
set_output_delay -add_delay -clock vclk -min 0 [get_ports {mem_a_n_lb}]
set_output_delay            -clock vclk -max 4 [get_ports {mem_a_n_ub}]
set_output_delay -add_delay -clock vclk -min 0 [get_ports {mem_a_n_ub}]

set_output_delay            -clock vclk -max 32 [get_ports {mem_a_addr*}]
set_output_delay -add_delay -clock vclk -min 0  [get_ports {mem_a_addr*}]
set_output_delay            -clock vclk -max 32 [get_ports {mem_a_data*}]
set_output_delay -add_delay -clock vclk -min 0  [get_ports {mem_a_data*}]
set_input_delay             -clock vclk -max 10 [get_ports {mem_a_data*}]
set_input_delay  -add_delay -clock vclk -min 0  [get_ports {mem_a_data*}]

set_output_delay            -clock vclk -max 18 [get_ports {usb_data*}]
set_output_delay -add_delay -clock vclk -min 0 [get_ports {usb_data*}]
set_input_delay             -clock vclk -max 10 [get_ports {usb_n_cmd}]
set_input_delay  -add_delay -clock vclk -min 0  [get_ports {usb_n_cmd}]
set_input_delay             -clock vclk -max 10 [get_ports {usb_n_frd}]
set_input_delay  -add_delay -clock vclk -min 0  [get_ports {usb_n_frd}]
set_input_delay             -clock vclk -max 10 [get_ports {usb_n_fwr}]
set_input_delay  -add_delay -clock vclk -min 0  [get_ports {usb_n_fwr}]
set_input_delay             -clock vclk -max 10 [get_ports {usb_data*}]
set_input_delay  -add_delay -clock vclk -min 0  [get_ports {usb_data*}]
