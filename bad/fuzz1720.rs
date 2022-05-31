
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1720(_: S3, _: S1, _: S7) {}
        
        fn test1720() { foo1720(S3, S5, S1, S6, S4, S5, S1); }
    