
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo6050(_: S4, _: S7, _: S5, _: S5) {}
        
        fn test6050() { foo6050(S1, S0, S4, S1, S3, S7); }
    