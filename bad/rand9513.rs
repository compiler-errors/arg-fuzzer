
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo9513(_: S2, _: S5, _: S6, _: S7) {}
        
        fn test9513() { foo9513(S5, S6, S0, S4, S3, S7, S2); }
    