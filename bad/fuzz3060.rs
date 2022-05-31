
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo3060(_: S7, _: S6, _: S5, _: S6, _: S5) {}
        
        fn test3060() { foo3060(S7, S4, S3, S2, S1, S5, S2, S8); }
    