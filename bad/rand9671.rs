
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo9671(_: S3, _: S4, _: S5, _: S7) {}
        
        fn test9671() { foo9671(S1, S1, S4, S0, S0, S1); }
    