
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1010(_: S3, _: S8, _: S2, _: S7) {}
        
        fn test1010() { foo1010(S6, S5, S3, S4, S4, S7, S8, S8); }
    