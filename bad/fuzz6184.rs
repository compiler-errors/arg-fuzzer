
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo6184(_: S1, _: S2, _: S4, _: S5, _: S6, _: S7, _: S8) {}
        
        fn test6184() { foo6184(S2, S3); }
    