
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo3910(_: S1, _: S4, _: S5, _: S7) {}
        
        fn test3910() { foo3910(S3, S5, S4, S1, S8); }
    