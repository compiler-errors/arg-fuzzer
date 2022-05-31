
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1678(_: S7, _: S1) {}
        
        fn test1678() { foo1678(S3, S4, S5, S6, S7, S8); }
    