import pyautogui
import mss
import numpy as np
import cv2
import torch

model = torch.hub.load('pytorch/vision:v0.10.0', 'resnet18', pretrained=True)

with mss.mss() as sct:
    while True:
        # Capture écran
        screenshot = sct.grab(sct.monitors[1])
        img = np.array(screenshot)
        img = cv2.cvtColor(img, cv2.COLOR_BGRA2BGR)
        # Save l'image
        cv2.imwrite('screenshot.png', img)