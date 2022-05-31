
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo9173(_: S5, _: S6, _: S7) {}
        
        fn test9173() { foo9173(S5, S1, S3, S6, S2, S8); }
    