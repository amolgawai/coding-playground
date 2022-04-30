from enum import Enum    

class MyEnum(str, Enum):
    state1 = 'state1'
    state2 = 'state2'

  
def main():
    print(MyEnum.state1.name, MyEnum.state2.name)


if __name__ == "__main__":
    main()
