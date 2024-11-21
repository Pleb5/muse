import torch

print(f"CUDA Available: {torch.cuda.is_available()}")
print(f"Device Name: {torch.cuda.get_device_name(0)}")

print(torch.cuda)
print(torch.cuda.is_available())
