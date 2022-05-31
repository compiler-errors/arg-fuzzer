
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo3094(_: S6, _: S7, _: S2, _: S3, _: S5, _: S1) {}
        
        fn test3094() { foo3094(S2, S0, S7, S4, S6, S0, S2); }
    