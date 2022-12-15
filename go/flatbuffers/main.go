package main

// $ go get github.com/google/flatbuffers/go

//go:generate flatc -o protoc --go proto/monster.fbs

import (
	"fmt"
	"goplay/flatbuffers/protoc/goplay/fb"
	"log"

	flatbuffers "github.com/google/flatbuffers/go"
)

func main(){
	if err := playFlatbuffers(); err != nil {
		log.Fatalf("playFlatbuffers() failed: %+v", err)
	}
}


func playFlatbuffers() error {
	// Create a monster
	b := flatbuffers.NewBuilder(1024)

	// Objects cannot be nested.
	// So we first create our strings and arrays, and then write the struct.
	name := b.CreateString("Sword")
	fb.WeaponStart(b)
	fb.WeaponAddName(b, name)
	fb.WeaponAddDamage(b, 3)
	sword := fb.WeaponEnd(b)

	// Create an array: inventory.
	fb.MonsterStartInventoryVector(b, 2)
	b.PrependByte(0)
	b.PrependByte(1)
	inv := b.EndVector(2)

	// Create an array: weapons. Build it, specify offset
	fb.MonsterStartWeaponsVector(b, 1)
	b.PrependUOffsetT(sword)
	weapons := b.EndVector(1)

	// Create an array: path
	fb.MonsterStartPathVector(b, 2)
	fb.CreateLocation(b, 1.0, 2.0, 3.0)
	fb.CreateLocation(b, 4.0, 5.0, 6.0)
	path := b.EndVector(2)

	// Create the monster

	fb.MonsterStart(b)
	fb.MonsterAddPosition(b, fb.CreateLocation(b, 1.0, 2.0, 3.0))
	fb.MonsterAddHp(b, 300)
	fb.MonsterAddColor(b, fb.ColorRed)

	// Add union: _type + value
	fb.MonsterAddEquippedType(b, fb.EquipmentWeapon)
	fb.MonsterAddEquipped(b, sword)
	
	// Add arrays
	fb.MonsterAddInventory(b, inv)
	fb.MonsterAddWeapons(b, weapons)
	fb.MonsterAddPath(b, path)

	// Done
	orc := fb.MonsterEnd(b)
	b.Finish(orc)

	// Serialize
	buf := b.FinishedBytes()
	fmt.Printf("Binary orc: %q\n", buf)
	
	// Unserialize
	monster := fb.GetRootAsMonster(buf, 0)
	fmt.Printf("Monster hp: %d\n", monster.Hp())

	return nil
}