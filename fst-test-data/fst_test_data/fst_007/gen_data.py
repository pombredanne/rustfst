#! /usr/bin/env python
# encoding: utf-8

from __future__ import unicode_literals

import os
import pynini as p
from fst_test_data.fst_test_data import FstTestData, weight_one


class FstTestData007(FstTestData):
    def get_raw_fst(self):
        fst = p.Fst(arc_type="standard")
        s0 = fst.add_state()
        s1 = fst.add_state()
        s2 = fst.add_state()
        s3 = fst.add_state()
        s4 = fst.add_state()

        fst.set_start(s0)
        fst.set_final(s4, 0.7)

        fst.add_arc(s0, p.Arc(12, 25, p.Weight(fst.weight_type(), 0.3), s1))
        fst.add_arc(s1, p.Arc(13, 26, p.Weight(fst.weight_type(), 0.4), s3))

        fst.add_arc(s0, p.Arc(12, 25, p.Weight(fst.weight_type(), 0.3), s2))
        fst.add_arc(s2, p.Arc(13, 26, p.Weight(fst.weight_type(), 0.4), s3))

        fst.add_arc(s3, p.Arc(14, 27, p.Weight(fst.weight_type(), 0.6), s4))

        return fst


if __name__ == "__main__":
    FstTestData007("fst_007", os.path.dirname(__file__)).compute_data()
