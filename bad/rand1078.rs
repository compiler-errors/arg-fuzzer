
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1078(_: S4, _: S7) {}
        
        fn test1078() { foo1078(S2, S5, S4, S2, S4); }
    