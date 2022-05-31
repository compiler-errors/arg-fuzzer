
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo14682(_: S2, _: S8, _: S6, _: S5) {}
        
        fn test14682() { foo14682(S4, S6, S6, S6, S2, S6); }
    