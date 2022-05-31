
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1503(_: S2, _: S6, _: S1, _: S3) {}
        
        fn test1503() { foo1503(S4, S7, S7, S2, S4, S8); }
    