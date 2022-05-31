
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1518(_: S2, _: S5, _: S6) {}
        
        fn test1518() { foo1518(S3, S5, S7, S4, S3, S2, S8); }
    