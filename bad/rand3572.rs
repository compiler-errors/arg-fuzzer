
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo3572(_: S2, _: S5, _: S6, _: S8) {}
        
        fn test3572() { foo3572(S5, S1, S6, S2, S0, S5); }
    