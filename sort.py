import os
import shutil

ANSWERS_FILE = "answers"
DIRS = ["mammograms/", "output/", "saturated/", "output_max_diff/", "saturated_max_diff/", "output_center_diff/", "saturated_center_diff/", "output_average/", "saturated_average/"]

for dir in DIRS:
    if not os.path.exists(dir + "Normal"):
        os.makedirs(dir + "Normal")
    if not os.path.exists(dir + "Abnormal"):
        os.makedirs(dir + "Abnormal")

with open(ANSWERS_FILE, "r") as answers_file:
    answers = answers_file.read().strip().split("\n")
    for line in answers:
        line = line.split(" ")
        image = line [0] + ".pgm"
        normal = line [2] == "NORM"
        for dir in DIRS:
            try:
                if normal:
                    shutil.move(dir + image, dir + "Normal/" + image)
                else:
                    shutil.move(dir + image, dir + "Abnormal/" + image)
            except:
                print("File " + dir + image + " not found")