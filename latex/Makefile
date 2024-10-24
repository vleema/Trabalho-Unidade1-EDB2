MAIN = main
OUTPUT_DIR = output

all: compile bibtex compile twice

compile:
	pdflatex -output-directory=$(OUTPUT_DIR) $(MAIN).tex

bibtex:
	bibtex $(OUTPUT_DIR)/$(MAIN)

twice:
	pdflatex -output-directory=$(OUTPUT_DIR) $(MAIN).tex
	pdflatex -output-directory=$(OUTPUT_DIR) $(MAIN).tex

clean:
	rm -f $(OUTPUT_DIR)/*.aux $(OUTPUT_DIR)/*.log $(OUTPUT_DIR)/*.bbl \
	      $(OUTPUT_DIR)/*.blg $(OUTPUT_DIR)/*.toc $(OUTPUT_DIR)/*.out \
	      $(OUTPUT_DIR)/*.lof $(OUTPUT_DIR)/*.lot $(OUTPUT_DIR)/*.synctex.gz
