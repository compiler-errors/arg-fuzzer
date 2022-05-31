
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo9657(_: S2, _: S4, _: S5, _: S7, _: S8) {}
        
        fn test9657() { foo9657(S0, S4, S6, S7, S4, S5, S6); }
    