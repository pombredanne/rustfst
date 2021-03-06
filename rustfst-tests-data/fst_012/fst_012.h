#ifndef FST_012
#define FST_012

class FstTestData012 {
public:
    using MyWeight = fst::TropicalWeight;
    using MyArc = fst::ArcTpl<MyWeight>;
    using MyFst = fst::VectorFst<MyArc>;

    FstTestData012() {}

    MyFst get_fst() const {
        auto parsed_fst = fst::ConstFst<MyArc>::Read(std::string("fst_012/hcl.fst.in"));
        MyFst f(*parsed_fst);
        delete parsed_fst;
        return f;
    }

    fst::VectorFst<MyArc> get_fst_compose() const {
        auto parsed_fst = fst::VectorFst<MyArc>::Read(std::string("fst_012/gp.fst.in"));
        MyFst f(*parsed_fst);
        delete parsed_fst;
        return f;
    }

    MyWeight get_weight_plus_mapper() const {
        return MyWeight(1.5);
    }

    MyWeight get_weight_times_mapper() const {
        return MyWeight(1.5);
    }

    fst::VectorFst<MyArc> get_fst_concat() const {
        return get_fst_compose();
    }

    fst::VectorFst<MyArc> get_fst_union() const {
        return get_fst_concat();
    }

    MyWeight random_weight() const {
        return MyWeight(custom_random_float());
    }
};


#endif