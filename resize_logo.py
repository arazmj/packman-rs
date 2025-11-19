from PIL import Image
import os

def resize_image(input_path, output_path, max_width=300):
    try:
        with Image.open(input_path) as img:
            # Calculate new height to maintain aspect ratio
            width_percent = (max_width / float(img.size[0]))
            hsize = int((float(img.size[1]) * float(width_percent)))
            
            # Resize
            img = img.resize((max_width, hsize), Image.Resampling.LANCZOS)
            
            # Save
            img.save(output_path, optimize=True, quality=85)
            print(f"Resized image saved to {output_path}")
            print(f"Original size: {os.path.getsize(input_path)} bytes")
            print(f"New size: {os.path.getsize(output_path)} bytes")
    except Exception as e:
        print(f"Error resizing image: {e}")

if __name__ == "__main__":
    resize_image("logo.png", "logo.png")
