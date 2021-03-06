###################################################################
# Project Configuration: 
# 
# Specify the name of the design (project), the Quartus II Settings
# File (.qsf), and the list of source files used.
###################################################################

DESIGN = risc16ba
PROJECT = $(DESIGN)_top
SOURCE_FILES = $(PROJECT).sv $(DESIGN).sv $(PROJECT).sdc
ASSIGNMENT_FILES = $(PROJECT).qpf $(PROJECT).qsf
TCL_FILE = $(PROJECT).tcl
SIM_TOP  = sim_$(DESIGN)
SIM_FILE = src/$(SIM_TOP).sv
SIM_WORK = work
SIM_DO   = "add wave -r /$(SIM_TOP)/$(DESIGN)_inst/* /$(SIM_TOP)/mem; run -all"
CHECKER  = check_$(SIM_TOP)
PGM_FILE = $(SIM_TOP).pgm
IMG_DUMP = $(SIM_TOP).dump
IMG_DIR  = imfiles
REF_DUMP = hist.dump
ASM_CODE = src/$(SIM_TOP).asm
BACKUP_TMP_DIR = $(shell date --date today +%m%d_%H%M%S)
FNAME = $(BACKUP_TMP_DIR).tar.bak
BACKUP_DIR = backups


###################################################################
# Main Targets
#
# all: build everything
# clean: remove output files and database
###################################################################

all: smart.log $(PROJECT).asm.rpt $(PROJECT).sta.rpt $(PROJECT).rbf

clean:
	rm -rf *.rbf *.rpt *.chg smart.log *.htm *.eqn *.pin *.sof *.jdi \
	*.pof db INCA_libs ncverilog.log incremental_db *~ $(CHECKER) \
	transcript qdb \#*\#

distclean: clean
	rm -rf $(ASSIGNMENT_FILES) *.summary *.smsg *.dpf *.qws
	rm -rf *.shm *.wlf *.log $(SIM_WORK) $(IMG_DUMP) $(PGM_FILE)

config: $(PROJECT).rbf
	PsiUsbConfig -f./$(PROJECT).rbf

map: smart.log $(PROJECT).map.rpt
fit: smart.log $(PROJECT).fit.rpt
asm: smart.log $(PROJECT).asm.rpt
sta: smart.log $(PROJECT).sta.rpt
smart: smart.log

###################################################################
# Executable Configuration
###################################################################

MAP_ARGS = --family=Cyclone
FIT_ARGS = --part=EP1C20F400C8
ASM_ARGS =
STA_ARGS =

###################################################################
# Target implementations
###################################################################

STAMP = echo done >

$(PROJECT).map.rpt: map.chg $(SOURCE_FILES) 
	quartus_map $(MAP_ARGS) $(PROJECT)
	$(STAMP) fit.chg

$(PROJECT).fit.rpt: fit.chg $(PROJECT).map.rpt
	quartus_fit $(FIT_ARGS) $(PROJECT)
	$(STAMP) asm.chg
	$(STAMP) sta.chg

$(PROJECT).asm.rpt: asm.chg $(PROJECT).fit.rpt
	quartus_asm $(ASM_ARGS) $(PROJECT)

$(PROJECT).rbf: asm.chg $(PROJECT).fit.rpt
	quartus_asm $(ASM_ARGS) $(PROJECT)

$(PROJECT).sta.rpt: sta.chg $(PROJECT).fit.rpt
	quartus_sta --do_report_timing $(STA_ARGS) $(PROJECT) 

smart.log: $(ASSIGNMENT_FILES)
	quartus_sh --determine_smart_action $(PROJECT) > smart.log

###################################################################
# Project initialization
###################################################################

$(ASSIGNMENT_FILES): $(TCL_FILE)
	quartus_sh -t $(TCL_FILE)

map.chg:
	$(STAMP) map.chg
fit.chg:
	$(STAMP) fit.chg
sta.chg:
	$(STAMP) sta.chg
asm.chg:
	$(STAMP) asm.chg

###################################################################
# Simulation
###################################################################
$(SIM_WORK):
	vlib work

$(IMG_DUMP): sims


$(SIM_TOP).mem: $(ASM_CODE)
	asm-risc16 $(ASM_CODE) -o $(SIM_TOP).mem

mem: $(ASM_CODE)
	asm-risc16 $(ASM_CODE) -o $(SIM_TOP).mem

sim: $(SIM_WORK) $(SIM_TOP).mem							      
	rm -f $(IMG_DUMP) $(PGM_FILE) 					      
	vlog $(SIM_FILE) src/$(DESIGN).sv				      
	vsim -c $(SIM_TOP) -l $(SIM_TOP).log -wlf $(SIM_TOP).wlf -do $(SIM_DO)

sims: $(SIM_WORK) $(SIM_TOP).mem
	rm -f $(IMG_DUMP) $(PGM_FILE)                                         
	vlog src/$(SIM_TOP)_s.sv src/$(DESIGN).sv                                         
	vsim -c $(SIM_TOP) -l $(SIM_TOP).log -wlf $(SIM_TOP).wlf -do $(SIM_DO)

$(BACKUP_DIR):
	mkdir $(BACKUP_DIR)


bak :  src/$(DESIGN).sv $(ASM_CODE) $(BACKUP_DIR) risc16ba_top.sta.rpt risc16ba_top.map.summary
	@cp risc16ba_top.sta.rpt ./src
	@cp risc16ba_top.map.summary ./src
	@tar cvf $(FNAME) src/*
	@echo backup archive $(FNAME) was created.
	@mv $(FNAME) $(BACKUP_DIR)

push:
	git add -A
	git commit -m "update"
	git push

check: $(CHECKER) $(IMG_DUMP) 
	./$(CHECKER) $(IMG_DUMP) $(PGM_FILE)
	diff $(IMG_DUMP) $(IMG_DIR)/$(REF_DUMP)

$(CHECKER): $(CHECKER).c
	gcc -o $@ -Wall -O2 $< -lm

spec:risc16ba_top.sta.rpt risc16ba_top.map.summary
	@cat risc16ba_top.sta.rpt | grep processor_clk | grep MHz | grep -v pll
	@cat risc16ba_top.map.summary | grep logic
