
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo3622(_: S1, _: S2, _: S3, _: S4, _: S5, _: S7) {}
        
        fn test3622() { foo3622(S5, S4, S8, S3, S2, S6, S1, S1); }
    