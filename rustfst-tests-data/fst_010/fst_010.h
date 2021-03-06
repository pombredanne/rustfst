#ifndef FST_010
#define FST_010

class FstTestData010 {
public:
    using MyArc = fst::LogArc;
    using MyWeight = MyArc::Weight;
    using MyFst = fst::VectorFst<MyArc>;

    FstTestData010() {}

    MyFst get_fst() const {
        fst::VectorFst<fst::LogArc> f;

        auto s0 = f.AddState();
        auto s1 = f.AddState();
        auto s2 = f.AddState();
        auto s3 = f.AddState();
        auto s4 = f.AddState();

        f.SetStart(s0);
        f.SetFinal(s3, 0.7);
        f.SetFinal(s4, 0.8);

        f.AddArc(s0, fst::LogArc(12, 12, 0.3, s1));
        f.AddArc(s1, fst::LogArc(13, 13, 0.4, s3));

        f.AddArc(s0, fst::LogArc(14, 14, 0.5, s2));
        f.AddArc(s2, fst::LogArc(15, 15, 0.6, s4));

        return f;
    }

    fst::VectorFst<MyArc> get_fst_compose() const {
        fst::VectorFst<MyArc> fst_2;
        fst_2.AddState();
        fst_2.AddState();
        fst_2.AddState();
        fst_2.SetStart(0);
        fst_2.SetFinal(2, MyWeight(1.2));
        fst_2.AddArc(0, MyArc(12, 2, MyWeight(1.7), 1));
        fst_2.AddArc(1, MyArc(13, 2, MyWeight(1.7), 2));
        return fst_2;
    }

    MyWeight get_weight_plus_mapper() const {
        return MyWeight(1.5);
    }

    MyWeight get_weight_times_mapper() const {
        return MyWeight(1.5);
    }

    fst::VectorFst<MyArc> get_fst_concat() const {
        fst::VectorFst<MyArc> fst_2;
        fst_2.AddState();
        fst_2.AddState();
        fst_2.AddState();
        fst_2.SetStart(0);
        fst_2.SetFinal(2, MyWeight(0.3));
        fst_2.AddArc(0, MyArc(2, 12, MyWeight(1.2), 1));
        fst_2.AddArc(0, MyArc(3, 1, MyWeight(2.2), 1));
        fst_2.AddArc(1, MyArc(6, 3, MyWeight(2.3), 2));
        fst_2.AddArc(1, MyArc(4, 2, MyWeight(1.7), 2));
        return fst_2;
    }

    fst::VectorFst<MyArc> get_fst_union() const {
        return get_fst_concat();
    }

    MyWeight random_weight() const {
        return MyWeight(custom_random_float());
    }
};

#endif