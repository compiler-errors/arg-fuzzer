
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo9465(_: S3, _: S4, _: S5, _: S3, _: S0, _: S4) {}
        
        fn test9465() { foo9465(S2, S7, S7, S5, S3, S1, S2); }
    