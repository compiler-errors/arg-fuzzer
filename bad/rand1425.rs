
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1425(_: S4, _: S7) {}
        
        fn test1425() { foo1425(S6, S4, S3, S5, S4); }
    