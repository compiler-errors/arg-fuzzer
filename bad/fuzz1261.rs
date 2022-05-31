
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1261(_: S1, _: S2) {}
        
        fn test1261() { foo1261(S5, S1, S4, S1, S5, S1, S4, S8); }
    