
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo6488(_: S6, _: S8) {}
        
        fn test6488() { foo6488(S1, S5, S6, S4, S2, S5); }
    