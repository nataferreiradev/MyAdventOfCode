from storage import Storage

def main():
   storage = Storage()

   storage.add(1)
   storage.add(3)
   storage.add("a")
   storage.add(1)

   print("="*10)
   print(storage.list)
   print(storage.map_index)
   storage.remove(1)
   print("="*10)
   print(storage.list)
   print(storage.map_index)
   print("="*10)

if __name__ == "__main__":
    main()
