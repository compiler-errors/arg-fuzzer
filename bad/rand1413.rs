
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1413(_: S1, _: S3, _: S5) {}
        
        fn test1413() { foo1413(S3, S8, S6, S7, S1, S4); }
    