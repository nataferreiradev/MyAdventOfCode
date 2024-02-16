class Storage():
    def __init__(self):
        self.list = []
        self.map_index = {}
   
    def add(self,value):
        self.list.append(value);
        if value in self.map_index:
            self.map_index[value].append(len(self.list)-1)
        else:
            self.map_index[value] = [len(self.list)-1]
    
    def remove(self, value):
        if value in self.map_index:
            for index in self.map_index[value]:
                last_index = len(self.list) - 1
                if index == last_index:
                    self.list.pop()
                else:
                    aux = self.list[index]
                    self.list[index] = self.list[last_index]
                    self.list[last_index] = aux
                    self.list.pop()
        del self.map_index[value]
