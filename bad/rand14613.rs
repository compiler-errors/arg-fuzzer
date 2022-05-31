
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo14613(_: S5, _: S0, _: S1) {}
        
        fn test14613() { foo14613(S2, S4, S5, S6, S8); }
    