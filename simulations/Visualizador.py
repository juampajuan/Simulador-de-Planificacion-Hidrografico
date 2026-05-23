import rasterio
import matplotlib.pyplot as plt
import numpy as np

tiff_path = "Darsena_20cm_v2.tif"
with rasterio.open(tiff_path) as src:
    img = src.read(1)

def read_points(filename):
    xs, ys = [], []
    with open(filename, "r") as f:
        for line in f:
            line = line.strip().replace("(", "").replace(")", "")
            parts = line.split(",")
            if len(parts) == 2:
                xs.append(int(parts[0]))
                ys.append(int(parts[1]))
    return xs, ys

def read_measures(filename):
    xs, ys, vals = [], [], []
    with open(filename, "r") as f:
        for line in f:
            parts = line.strip().split(",")
            if len(parts) == 3:
                xs.append(int(parts[0]))
                ys.append(int(parts[1]))
                vals.append(float(parts[2]))
    return xs, ys, vals

x_coords, y_coords   = read_points("res.txt")
x_points, y_points   = read_points("points.txt")
#x_perp,   y_perp     = read_points("perp.txt")
#x_circ,   y_circ     = read_points("circ.txt")

#x_mperp, y_mperp, vals_perp   = read_measures("measures_perp.txt")
#x_mcirc, y_mcirc, vals_circle = read_measures("measures_circle.txt")

# ── Figura 1: Recorrido y puntos de muestreo ──────────────────────────────────
fig1, ax1 = plt.subplots(figsize=(10, 10))
ax1.imshow(img, cmap='viridis')
ax1.plot(x_coords, y_coords, color='red', linewidth=1.5, marker='.', markersize=2, label='Recorrido')
#ax1.scatter(x_perp,   y_perp,   color='lime',  marker='o', s=8,  label='Puntos perpendiculares')
#ax1.scatter(x_circ,   y_circ,   color='cyan',  marker='o', s=8,  label='Puntos circulares')
ax1.scatter(x_points, y_points, color='green', marker='o', s=20, label='Puntos de medición')
ax1.set_title("Recorrido y puntos de muestreo")
ax1.legend(loc='upper right')

# ── Figura 2: Medidas perpendiculares ─────────────────────────────────────────
# fig2, ax2 = plt.subplots(figsize=(10, 10))
# ax2.imshow(img, cmap='gray', alpha=0.6)
# sc2 = ax2.scatter(x_mperp, y_mperp, c=vals_perp, cmap='plasma', marker='o', s=20)
# plt.colorbar(sc2, ax=ax2, label='Profundidad promedio')
# ax2.plot(x_coords, y_coords, color='red', linewidth=1, alpha=0.4)
# ax2.set_title("Medidas — modo perpendicular")

# ── Figura 3: Medidas circulares ──────────────────────────────────────────────
# fig3, ax3 = plt.subplots(figsize=(10, 10))
# ax3.imshow(img, cmap='gray', alpha=0.6)
# sc3 = ax3.scatter(x_mcirc, y_mcirc, c=vals_circle, cmap='plasma', marker='o', s=20)
# plt.colorbar(sc3, ax=ax3, label='Profundidad promedio')
# ax3.plot(x_coords, y_coords, color='red', linewidth=1, alpha=0.4)
# ax3.set_title("Medidas — modo circular")


plt.show()