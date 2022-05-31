
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo3269(_: S2, _: S4) {}
        
        fn test3269() { foo3269(S1, S2, S5, S6, S8); }
    