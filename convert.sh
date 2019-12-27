declare -a folders=("output" "output_average" "output_average_div16" "output_center_diff" "output_center_diff_div16" "output_div16" "output_max_diff" "output_max_diff_div16" "saturated" "saturated_div16" "saturated_output" "saturated_output_average" "saturated_output_average_div16" "saturated_output_center_diff" "saturated_output_center_diff_div16" "saturated_output_div16" "saturated_output_max_diff" "saturated_output_max_diff_div16" "base" "base_div16")

for i in "${folders[@]}"
do
    echo "$i"
    cd "$i"
    cd Normal/
    mogrify -format bmp *.pgm
    rm *.pgm
    cd ..
    cd Abnormal/
    mogrify -format bmp *.pgm
    rm *.pgm
    cd ..
    cd ..
done
