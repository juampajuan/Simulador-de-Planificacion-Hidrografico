import rasterio
import matplotlib.pyplot as plt

tiff_path = "simulador-hidrografico/Darsena_20cm_v2.tif"
with rasterio.open(tiff_path) as src:
    img = src.read(1)
    
x_coords = []
y_coords = []
with open("simulador-hidrografico/res.txt", "r") as f:
    for line in f:
        line = line.strip().replace("(", "").replace(")", "")
        parts = line.split(",")
        if len(parts) == 2:
            x_coords.append(int(parts[0]))
            y_coords.append(int(parts[1]))


x_points = []
y_points = []
with open("simulador-hidrografico/points.txt", "r") as f:
    for line in f:
        line = line.strip().replace("(", "").replace(")", "")
        parts = line.split(",")
        if len(parts) == 2:
            x_points.append(int(parts[0]))
            y_points.append(int(parts[1]))
            
plt.figure(figsize=(10, 10))

plt.imshow(img, cmap='viridis') 

# Dibuja el recorrido en rojo
plt.plot(x_coords, y_coords, color='red', linewidth=1.5, marker='.', markersize=2)
plt.scatter(x_points, y_points, color='green', marker='o')
plt.title("Simulación de Recorrido Batimétrico")
plt.show()