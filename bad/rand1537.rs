
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1537(_: S1, _: S2, _: S4, _: S5, _: S6) {}
        
        fn test1537() { foo1537(S7, S2, S0, S1, S0, S7, S6); }
    