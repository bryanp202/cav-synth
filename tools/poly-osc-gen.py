osc_count = 16

for i in range(1, osc_count + 1):
    print(f"// Osc{i}")
    print(f"Cable::new(0, {i*3}, {osc_count+i}, 0),")
    print(f"Cable::new(0, {i*3 + 2}, {osc_count+i}, 1),")
    print(f"Cable::new({osc_count+i}, 0, {i}, 0),")
    print(f"Cable::new(0, {i*3 + 1}, {i}, 1),")
    print(f"Cable::new({i}, 0, {osc_count*2+i}, 0),")
    # print(f"Cable::new(0, {i*3+1}, {osc_count*2+i}, 1),")
    print(f"Cable::new({osc_count+i}, 0, {osc_count*2+i}, 1),")
    print(f"Cable::new({osc_count*2+i}, 0, {osc_count*3 + 1}, 0),")

print("\n// Chorus to delay")
print(f"Cable::new({osc_count*3 + 1}, 0, {osc_count*3 + 2}, 0),")