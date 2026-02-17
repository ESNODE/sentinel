# Sentinel Mock Temperature Skill (Python)
# This skill simulates a liquid cooling temperature sensor.
import random

def collect():
    # Simulate a sensor reading
    inlet_temp = 22.0 + random.uniform(-0.5, 0.5)
    outlet_temp = 28.0 + random.uniform(-1.0, 1.0)
    
    # In a real Sentinel WASM skill, you would call:
    # report_metric("mock_liquid_inlet_temp", inlet_temp)
    # report_metric("mock_liquid_outlet_temp", outlet_temp)
    
    print(f"Skill Executed: Inlet={inlet_temp:.2f}C, Outlet={outlet_temp:.2f}C")

if __name__ == "__main__":
    collect()
