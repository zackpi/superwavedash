#!/bin/sh

home=~zpitc

docs=$home'/Documents'
desk=$home'/Desktop'

mmw=$docs'/MMW'

echo $docs
echo $desk

stage=$desk'/stages/GrNBa.dat'
vanilla=$docs'/roots/vanilla.iso'
dirty=$docs'/roots/dirty.iso'

echo $stage
echo $vanilla
echo $dirty

cd $mmw
python main.py test --boot $stage --discPath $vanilla

