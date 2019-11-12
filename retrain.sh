declare -a folders=("output" "output_average" "output_average_div16" "output_center_diff" "output_center_diff_div16" "output_div16" "output_max_diff" "output_max_diff_div16" "saturated" "saturated_div16" "saturated_output" "saturated_output_average" "saturated_output_average_div16" "saturated_output_center_diff" "saturated_output_center_diff_div16" "saturated_output_div16" "saturated_output_max_diff" "saturated_output_max_diff_div16" "base" "base_div16")

for i in "${folders[@]}"
do
    python3 -m scripts.retrain \
            --bottleneck_dir=tf_files/bottlenecks \
            --model_dir=tf_files/models/"${ARCHITECTURE}" \
            --summaries_dir=tf_files/training_summaries/"${ARCHITECTURE}" \
            --output_graph=tf_files/"$i"_graph.pb \
            --output_labels=tf_files/"$i"_labels.txt \
            --image_dir=tf_files/"$i" \
            2>&1 | tee "$i".out
done
